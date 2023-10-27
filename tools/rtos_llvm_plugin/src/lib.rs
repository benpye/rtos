use std::marker::PhantomData;

use llvm_plugin::{
    inkwell::{
        module::{FlagBehavior, Module},
        values::GlobalValue,
    },
    LlvmModulePass, PassBuilder, PipelineParsing, PreservedAnalyses,
};
use llvm_sys::prelude::*;

#[llvm_plugin::plugin(name = "RtosPlugin", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_module_pipeline_parsing_callback(|name, manager| {
        if name == "set-small-data-limit" {
            manager.add_pass(SetSmallDataLimit);
            PipelineParsing::Parsed
        } else if name == "rtos-rewrite-symbols" {
            manager.add_pass(RewriteSymbols);
            PipelineParsing::Parsed
        } else {
            PipelineParsing::NotParsed
        }
    });
}

struct RewriteSymbols;
impl LlvmModulePass for RewriteSymbols {
    fn run_pass(
        &self,
        module: &mut llvm_plugin::inkwell::module::Module<'_>,
        _manager: &llvm_plugin::ModuleAnalysisManager,
    ) -> PreservedAnalyses {
        let component_name = std::env::var_os("RTOS_COMPONENT").unwrap();
        let component_suffix = format!(".{}", component_name.to_str().unwrap());

        let rename = |gv: GlobalValue<'_>| {
            let mut name = gv.get_name().to_str().unwrap().to_string();

            // Don't rename llvm intrinsics or rtos globals.
            if name.starts_with("llvm.") || name.starts_with("rtos.") {
                return;
            }

            name.push_str(&component_suffix);
            gv.set_name(&name);
        };

        module
            .get_functions()
            .map(|f| f.as_global_value())
            .for_each(rename);
        module.get_globals().for_each(rename);
        GlobalAliasIterator::from_module(module).for_each(rename);

        PreservedAnalyses::None
    }
}

struct GlobalAliasIterator<'ctx> {
    global_alias: LLVMValueRef,
    _marker: PhantomData<&'ctx ()>,
}

impl<'ctx> GlobalAliasIterator<'ctx> {
    fn from_module(module: &Module<'ctx>) -> Self {
        GlobalAliasIterator {
            global_alias: unsafe { llvm_sys::core::LLVMGetFirstGlobalAlias(module.as_mut_ptr()) },
            _marker: PhantomData,
        }
    }
}

impl<'ctx> Iterator for GlobalAliasIterator<'ctx> {
    type Item = GlobalValue<'ctx>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.global_alias.is_null() {
            None
        } else {
            // This hack assumes that GlobalValue is just a wrapper around LLVMValueRef
            let value = unsafe { core::mem::transmute(self.global_alias) };
            self.global_alias =
                unsafe { llvm_sys::core::LLVMGetNextGlobalAlias(self.global_alias) };
            Some(value)
        }
    }
}

struct SetSmallDataLimit;
impl LlvmModulePass for SetSmallDataLimit {
    fn run_pass(
        &self,
        module: &mut llvm_plugin::inkwell::module::Module<'_>,
        _manager: &llvm_plugin::ModuleAnalysisManager,
    ) -> PreservedAnalyses {
        let ctx = module.get_context();
        let i32_type = ctx.i32_type();
        let i32_zero = i32_type.const_zero();
        let md_node = ctx.metadata_node(&[i32_zero.into()]);
        module.add_metadata_flag("SmallDataLimit", FlagBehavior::Error, md_node);
        PreservedAnalyses::None
    }
}

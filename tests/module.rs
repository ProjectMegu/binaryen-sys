#[test]
fn test_module() {
    unsafe {
        let module = binaryen_capi_sys::BinaryenModuleCreate();
        binaryen_capi_sys::BinaryenModulePrint(module);
        binaryen_capi_sys::BinaryenModuleDispose(module);
    }
}
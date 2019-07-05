
let wasm;

/**
* @returns {number}
*/
export function get_input_buffer_pointer() {
    return wasm.get_input_buffer_pointer();
}

/**
* @returns {number}
*/
export function get_output_buffer_pointer() {
    return wasm.get_output_buffer_pointer();
}

/**
* @param {number} index
* @param {number} value
* @returns {number}
*/
export function set_output_buffer(index, value) {
    return wasm.set_output_buffer(index, value);
}

/**
* @returns {void}
*/
export function hqx() {
    return wasm.hqx();
}

/**
* @returns {number}
*/
export function get_input_buffer_size() {
    return wasm.get_input_buffer_size() >>> 0;
}

/**
* @returns {number}
*/
export function get_output_buffer_size() {
    return wasm.get_output_buffer_size() >>> 0;
}

function init(module) {
    if (typeof module === 'undefined') {
        module = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    let result;
    const imports = {};

    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;


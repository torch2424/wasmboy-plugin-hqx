/* tslint:disable */
/**
* @returns {number} 
*/
export function get_input_buffer_pointer(): number;
/**
* @returns {number} 
*/
export function get_output_buffer_pointer(): number;
/**
* @param {number} index 
* @param {number} value 
* @returns {number} 
*/
export function set_output_buffer(index: number, value: number): number;
/**
* @returns {void} 
*/
export function hqx(): void;
/**
* @returns {number} 
*/
export function get_input_buffer_size(): number;
/**
* @returns {number} 
*/
export function get_output_buffer_size(): number;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        
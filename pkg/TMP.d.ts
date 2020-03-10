/* tslint:disable */
/* eslint-disable */
/**
* @param {number} id 
* @returns {any} 
*/
export function get_player(id: number): any;
/**
* @returns {any} 
*/
export function get_servers(): any;
/**
* @returns {any} 
*/
export function get_game_info(): any;

/**
* If `module_or_path` is {RequestInfo}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {RequestInfo | BufferSource | WebAssembly.Module} module_or_path
*
* @returns {Promise<any>}
*/
export default function init (module_or_path?: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;
        
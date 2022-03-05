

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}
/**
* @param {Uint8Array} pdf_data
* @returns {any}
*/
export function extract_pdf(pdf_data) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        var ptr0 = passArray8ToWasm0(pdf_data, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.extract_pdf(retptr, ptr0, len0);
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        var r2 = getInt32Memory0()[retptr / 4 + 2];
        if (r2) {
            throw takeObject(r1);
        }
        return takeObject(r0);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
    }
}

const imports = {
    __wbindgen_placeholder__: {
        __wbindgen_object_drop_ref: function(arg0) {
            takeObject(arg0);
        },
        __wbindgen_string_new: function(arg0, arg1) {
            var ret = getStringFromWasm0(arg0, arg1);
            return addHeapObject(ret);
        },
        __wbindgen_object_clone_ref: function(arg0) {
            var ret = getObject(arg0);
            return addHeapObject(ret);
        },
        __wbg_set_fbb49ad265f9dee8: function(arg0, arg1, arg2) {
            getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
        },
        __wbg_new_16f24b0728c5e67b: function() {
            var ret = new Array();
            return addHeapObject(ret);
        },
        __wbg_new_d3138911a89329b0: function() {
            var ret = new Object();
            return addHeapObject(ret);
        },
        __wbg_push_a72df856079e6930: function(arg0, arg1) {
            var ret = getObject(arg0).push(getObject(arg1));
            return ret;
        },
        __wbg_getTime_f8ce0ff902444efb: function(arg0) {
            var ret = getObject(arg0).getTime();
            return ret;
        },
        __wbg_new0_57a6a2c2aaed3fc5: function() {
            var ret = new Date();
            return addHeapObject(ret);
        },
        __wbindgen_throw: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
    },

};

const wasm_url = new URL('linsen_pdf_extractor_bg.wasm', import.meta.url);
let wasmCode = '';
switch (wasm_url.protocol) {
    case 'file:':
    wasmCode = await Deno.readFile(wasm_url);
    break
    case 'https:':
    case 'http:':
    wasmCode = await (await fetch(wasm_url)).arrayBuffer();
    break
    default:
    throw new Error(`Unsupported protocol: ${wasm_url.protocol}`);
    break
}

const wasmInstance = (await WebAssembly.instantiate(wasmCode, imports)).instance;
const wasm = wasmInstance.exports;

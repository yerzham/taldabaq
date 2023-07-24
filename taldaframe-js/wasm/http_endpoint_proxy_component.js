import { fetch } from '../target/fetch.js';

const base64Compile = str => WebAssembly.compile(typeof Buffer !== 'undefined' ? Buffer.from(str, 'base64') : Uint8Array.from(atob(str), b => b.charCodeAt(0)));

function clampGuest(i, min, max) {
  if (i < min || i > max) throw new TypeError(`must be between ${min} and ${max}`);
  return i;
}

class ComponentError extends Error {
  constructor (value) {
    const enumerable = typeof value !== 'string';
    super(enumerable ? `${String(value)} (see error.payload)` : value);
    Object.defineProperty(this, 'payload', { value, enumerable });
  }
}

let dv = new DataView(new ArrayBuffer());
const dataView = mem => dv.buffer === mem.buffer ? dv : dv = new DataView(mem.buffer);

const isNode = typeof process !== 'undefined' && process.versions && process.versions.node;
let _fs;
async function fetchCompile (url) {
  if (isNode) {
    _fs = _fs || await import('fs/promises');
    return WebAssembly.compile(await _fs.readFile(url));
  }
  return fetch(url).then(WebAssembly.compileStreaming);
}

function getErrorPayload(e) {
  if (e && hasOwnProperty.call(e, 'payload')) return e.payload;
  return e;
}

const hasOwnProperty = Object.prototype.hasOwnProperty;

const instantiateCore = WebAssembly.instantiate;

function toUint16(val) {
  val >>>= 0;
  val %= 2 ** 16;
  return val;
}

const utf8Decoder = new TextDecoder();

const utf8Encoder = new TextEncoder();

let utf8EncodedLen = 0;
function utf8Encode(s, realloc, memory) {
  if (typeof s !== 'string') throw new TypeError('expected a string');
  if (s.length === 0) {
    utf8EncodedLen = 0;
    return 1;
  }
  let allocLen = 0;
  let ptr = 0;
  let writtenTotal = 0;
  while (s.length > 0) {
    ptr = realloc(ptr, allocLen, 1, allocLen + s.length);
    allocLen += s.length;
    const { read, written } = utf8Encoder.encodeInto(
    s,
    new Uint8Array(memory.buffer, ptr + writtenTotal, allocLen - writtenTotal),
    );
    writtenTotal += written;
    s = s.slice(read);
  }
  if (allocLen > writtenTotal)
  ptr = realloc(ptr, allocLen, 1, writtenTotal);
  utf8EncodedLen = writtenTotal;
  return ptr;
}

let exports0;
let exports1;
let memory0;
let realloc0;

function lowering0(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
  const ptr0 = arg0;
  const len0 = arg1;
  const result0 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr0, len0));
  let variant2;
  switch (arg2) {
    case 0: {
      variant2= {
        tag: 'get',
      };
      break;
    }
    case 1: {
      variant2= {
        tag: 'head',
      };
      break;
    }
    case 2: {
      variant2= {
        tag: 'post',
      };
      break;
    }
    case 3: {
      variant2= {
        tag: 'put',
      };
      break;
    }
    case 4: {
      variant2= {
        tag: 'delete',
      };
      break;
    }
    case 5: {
      variant2= {
        tag: 'connect',
      };
      break;
    }
    case 6: {
      variant2= {
        tag: 'options',
      };
      break;
    }
    case 7: {
      variant2= {
        tag: 'trace',
      };
      break;
    }
    case 8: {
      variant2= {
        tag: 'patch',
      };
      break;
    }
    case 9: {
      const ptr1 = arg3;
      const len1 = arg4;
      const result1 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr1, len1));
      variant2= {
        tag: 'other',
        val: result1
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for Method');
    }
  }
  const len5 = arg6;
  const base5 = arg5;
  const result5 = [];
  for (let i = 0; i < len5; i++) {
    const base = base5 + i * 16;
    const ptr3 = dataView(memory0).getInt32(base + 0, true);
    const len3 = dataView(memory0).getInt32(base + 4, true);
    const result3 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr3, len3));
    const ptr4 = dataView(memory0).getInt32(base + 8, true);
    const len4 = dataView(memory0).getInt32(base + 12, true);
    const result4 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr4, len4));
    result5.push([result3, result4]);
  }
  let variant7;
  switch (arg7) {
    case 0: {
      variant7 = null;
      break;
    }
    case 1: {
      const ptr6 = arg8;
      const len6 = arg9;
      const result6 = new Uint8Array(memory0.buffer.slice(ptr6, ptr6 + len6 * 1));
      variant7 = result6;
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for option');
    }
  }
  let ret;
  try {
    ret = { tag: 'ok', val: fetch({
      path: result0,
      method: variant2,
      headers: result5,
      body: variant7,
    }) };
  } catch (e) {
    ret = { tag: 'err', val: getErrorPayload(e) };
  }
  const variant20 = ret;
  switch (variant20.tag) {
    case 'ok': {
      const e = variant20.val;
      dataView(memory0).setInt8(arg10 + 0, 0, true);
      const {statusCode: v8_0, headers: v8_1, body: v8_2 } = e;
      dataView(memory0).setInt16(arg10 + 4, toUint16(v8_0), true);
      const vec12 = v8_1;
      const len12 = vec12.length;
      const result12 = realloc0(0, 0, 4, len12 * 16);
      for (let i = 0; i < vec12.length; i++) {
        const e = vec12[i];
        const base = result12 + i * 16;const [tuple9_0, tuple9_1] = e;
        const ptr10 = utf8Encode(tuple9_0, realloc0, memory0);
        const len10 = utf8EncodedLen;
        dataView(memory0).setInt32(base + 4, len10, true);
        dataView(memory0).setInt32(base + 0, ptr10, true);
        const ptr11 = utf8Encode(tuple9_1, realloc0, memory0);
        const len11 = utf8EncodedLen;
        dataView(memory0).setInt32(base + 12, len11, true);
        dataView(memory0).setInt32(base + 8, ptr11, true);
      }
      dataView(memory0).setInt32(arg10 + 12, len12, true);
      dataView(memory0).setInt32(arg10 + 8, result12, true);
      const variant14 = v8_2;
      if (variant14 === null || variant14=== undefined) {
        dataView(memory0).setInt8(arg10 + 16, 0, true);
      } else {
        const e = variant14;
        dataView(memory0).setInt8(arg10 + 16, 1, true);
        const val13 = e;
        const len13 = val13.byteLength;
        const ptr13 = realloc0(0, 0, 1, len13 * 1);
        const src13 = new Uint8Array(val13.buffer || val13, val13.byteOffset, len13 * 1);
        (new Uint8Array(memory0.buffer, ptr13, len13 * 1)).set(src13);
        dataView(memory0).setInt32(arg10 + 24, len13, true);
        dataView(memory0).setInt32(arg10 + 20, ptr13, true);
      }
      break;
    }
    case 'err': {
      const e = variant20.val;
      dataView(memory0).setInt8(arg10 + 0, 1, true);
      const variant19 = e;
      switch (variant19.tag) {
        case 'invalid-url': {
          const e = variant19.val;
          dataView(memory0).setInt8(arg10 + 4, 0, true);
          const ptr15 = utf8Encode(e, realloc0, memory0);
          const len15 = utf8EncodedLen;
          dataView(memory0).setInt32(arg10 + 12, len15, true);
          dataView(memory0).setInt32(arg10 + 8, ptr15, true);
          break;
        }
        case 'timeout-error': {
          const e = variant19.val;
          dataView(memory0).setInt8(arg10 + 4, 1, true);
          const ptr16 = utf8Encode(e, realloc0, memory0);
          const len16 = utf8EncodedLen;
          dataView(memory0).setInt32(arg10 + 12, len16, true);
          dataView(memory0).setInt32(arg10 + 8, ptr16, true);
          break;
        }
        case 'protocol-error': {
          const e = variant19.val;
          dataView(memory0).setInt8(arg10 + 4, 2, true);
          const ptr17 = utf8Encode(e, realloc0, memory0);
          const len17 = utf8EncodedLen;
          dataView(memory0).setInt32(arg10 + 12, len17, true);
          dataView(memory0).setInt32(arg10 + 8, ptr17, true);
          break;
        }
        case 'unexpected-error': {
          const e = variant19.val;
          dataView(memory0).setInt8(arg10 + 4, 3, true);
          const ptr18 = utf8Encode(e, realloc0, memory0);
          const len18 = utf8EncodedLen;
          dataView(memory0).setInt32(arg10 + 12, len18, true);
          dataView(memory0).setInt32(arg10 + 8, ptr18, true);
          break;
        }
        default: {
          throw new TypeError('invalid variant specified for Error');
        }
      }
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for result');
    }
  }
}
let exports2;
let postReturn0;
let postReturn1;

function handleRequest(arg0) {
  const {path: v0_0, method: v0_1, headers: v0_2, body: v0_3 } = arg0;
  const ptr1 = utf8Encode(v0_0, realloc0, memory0);
  const len1 = utf8EncodedLen;
  const variant3 = v0_1;
  let variant3_0;
  let variant3_1;
  let variant3_2;
  switch (variant3.tag) {
    case 'get': {
      variant3_0 = 0;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'head': {
      variant3_0 = 1;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'post': {
      variant3_0 = 2;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'put': {
      variant3_0 = 3;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'delete': {
      variant3_0 = 4;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'connect': {
      variant3_0 = 5;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'options': {
      variant3_0 = 6;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'trace': {
      variant3_0 = 7;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'patch': {
      variant3_0 = 8;
      variant3_1 = 0;
      variant3_2 = 0;
      break;
    }
    case 'other': {
      const e = variant3.val;
      const ptr2 = utf8Encode(e, realloc0, memory0);
      const len2 = utf8EncodedLen;
      variant3_0 = 9;
      variant3_1 = ptr2;
      variant3_2 = len2;
      break;
    }
    default: {
      throw new TypeError('invalid variant specified for Method');
    }
  }
  const vec7 = v0_2;
  const len7 = vec7.length;
  const result7 = realloc0(0, 0, 4, len7 * 16);
  for (let i = 0; i < vec7.length; i++) {
    const e = vec7[i];
    const base = result7 + i * 16;const [tuple4_0, tuple4_1] = e;
    const ptr5 = utf8Encode(tuple4_0, realloc0, memory0);
    const len5 = utf8EncodedLen;
    dataView(memory0).setInt32(base + 4, len5, true);
    dataView(memory0).setInt32(base + 0, ptr5, true);
    const ptr6 = utf8Encode(tuple4_1, realloc0, memory0);
    const len6 = utf8EncodedLen;
    dataView(memory0).setInt32(base + 12, len6, true);
    dataView(memory0).setInt32(base + 8, ptr6, true);
  }
  const variant9 = v0_3;
  let variant9_0;
  let variant9_1;
  let variant9_2;
  if (variant9 === null || variant9=== undefined) {
    variant9_0 = 0;
    variant9_1 = 0;
    variant9_2 = 0;
  } else {
    const e = variant9;
    const val8 = e;
    const len8 = val8.byteLength;
    const ptr8 = realloc0(0, 0, 1, len8 * 1);
    const src8 = new Uint8Array(val8.buffer || val8, val8.byteOffset, len8 * 1);
    (new Uint8Array(memory0.buffer, ptr8, len8 * 1)).set(src8);
    variant9_0 = 1;
    variant9_1 = ptr8;
    variant9_2 = len8;
  }
  const ret = exports1['taldawasm:main/http-endpoint#handle-request'](ptr1, len1, variant3_0, variant3_1, variant3_2, result7, len7, variant9_0, variant9_1, variant9_2);
  let variant20;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      const len12 = dataView(memory0).getInt32(ret + 12, true);
      const base12 = dataView(memory0).getInt32(ret + 8, true);
      const result12 = [];
      for (let i = 0; i < len12; i++) {
        const base = base12 + i * 16;
        const ptr10 = dataView(memory0).getInt32(base + 0, true);
        const len10 = dataView(memory0).getInt32(base + 4, true);
        const result10 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr10, len10));
        const ptr11 = dataView(memory0).getInt32(base + 8, true);
        const len11 = dataView(memory0).getInt32(base + 12, true);
        const result11 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr11, len11));
        result12.push([result10, result11]);
      }
      let variant14;
      switch (dataView(memory0).getUint8(ret + 16, true)) {
        case 0: {
          variant14 = null;
          break;
        }
        case 1: {
          const ptr13 = dataView(memory0).getInt32(ret + 20, true);
          const len13 = dataView(memory0).getInt32(ret + 24, true);
          const result13 = new Uint8Array(memory0.buffer.slice(ptr13, ptr13 + len13 * 1));
          variant14 = result13;
          break;
        }
        default: {
          throw new TypeError('invalid variant discriminant for option');
        }
      }
      variant20= {
        tag: 'ok',
        val: {
          statusCode: clampGuest(dataView(memory0).getUint16(ret + 4, true), 0, 65535),
          headers: result12,
          body: variant14,
        }
      };
      break;
    }
    case 1: {
      let variant19;
      switch (dataView(memory0).getUint8(ret + 4, true)) {
        case 0: {
          const ptr15 = dataView(memory0).getInt32(ret + 8, true);
          const len15 = dataView(memory0).getInt32(ret + 12, true);
          const result15 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr15, len15));
          variant19= {
            tag: 'invalid-url',
            val: result15
          };
          break;
        }
        case 1: {
          const ptr16 = dataView(memory0).getInt32(ret + 8, true);
          const len16 = dataView(memory0).getInt32(ret + 12, true);
          const result16 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr16, len16));
          variant19= {
            tag: 'timeout-error',
            val: result16
          };
          break;
        }
        case 2: {
          const ptr17 = dataView(memory0).getInt32(ret + 8, true);
          const len17 = dataView(memory0).getInt32(ret + 12, true);
          const result17 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr17, len17));
          variant19= {
            tag: 'protocol-error',
            val: result17
          };
          break;
        }
        case 3: {
          const ptr18 = dataView(memory0).getInt32(ret + 8, true);
          const len18 = dataView(memory0).getInt32(ret + 12, true);
          const result18 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr18, len18));
          variant19= {
            tag: 'unexpected-error',
            val: result18
          };
          break;
        }
        default: {
          throw new TypeError('invalid variant discriminant for Error');
        }
      }
      variant20= {
        tag: 'err',
        val: variant19
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  postReturn0(ret);
  if (variant20.tag === 'err') {
    throw new ComponentError(variant20.val);
  }
  return variant20.val;
}

function handleMessage(arg0) {
  const ptr0 = utf8Encode(arg0, realloc0, memory0);
  const len0 = utf8EncodedLen;
  const ret = exports1['taldawasm:main/mqtt-endpoint#handle-message'](ptr0, len0);
  let variant3;
  switch (dataView(memory0).getUint8(ret + 0, true)) {
    case 0: {
      const ptr1 = dataView(memory0).getInt32(ret + 4, true);
      const len1 = dataView(memory0).getInt32(ret + 8, true);
      const result1 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr1, len1));
      variant3= {
        tag: 'ok',
        val: result1
      };
      break;
    }
    case 1: {
      const ptr2 = dataView(memory0).getInt32(ret + 4, true);
      const len2 = dataView(memory0).getInt32(ret + 8, true);
      const result2 = utf8Decoder.decode(new Uint8Array(memory0.buffer, ptr2, len2));
      variant3= {
        tag: 'err',
        val: result2
      };
      break;
    }
    default: {
      throw new TypeError('invalid variant discriminant for expected');
    }
  }
  postReturn1(ret);
  if (variant3.tag === 'err') {
    throw new ComponentError(variant3.val);
  }
  return variant3.val;
}

const $init = (async() => {
  const module0 = fetchCompile(new URL('./http_endpoint_proxy_component.core.wasm', import.meta.url));
  const module1 = base64Compile('AGFzbQEAAAABDwFgC39/f39/f39/f39/AAMCAQAEBQFwAQEBBxACATAAAAgkaW1wb3J0cwEACh8BHQAgACABIAIgAyAEIAUgBiAHIAggCSAKQQARAAALAC4JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQGMC4xMy4wAEoEbmFtZQATEndpdC1jb21wb25lbnQ6c2hpbQEuAQAraW5kaXJlY3QtdGFsZGF3YXNtOm1haW4vaHR0cC1vdXRnb2luZy1mZXRjaA');
  const module2 = base64Compile('AGFzbQEAAAABDwFgC39/f39/f39/f39/AAIVAgABMAAAAAgkaW1wb3J0cwFwAQEBCQcBAEEACwEAAC4JcHJvZHVjZXJzAQxwcm9jZXNzZWQtYnkBDXdpdC1jb21wb25lbnQGMC4xMy4wABwEbmFtZQAVFHdpdC1jb21wb25lbnQ6Zml4dXBz');
  Promise.all([module0, module1, module2]).catch(() => {});
  ({ exports: exports0 } = await instantiateCore(await module1));
  ({ exports: exports1 } = await instantiateCore(await module0, {
    'taldawasm:main/http-outgoing': {
      fetch: exports0['0'],
    },
  }));
  memory0 = exports1.memory;
  realloc0 = exports1.cabi_realloc;
  ({ exports: exports2 } = await instantiateCore(await module2, {
    '': {
      $imports: exports0.$imports,
      '0': lowering0,
    },
  }));
  postReturn0 = exports1['cabi_post_taldawasm:main/http-endpoint#handle-request'];
  postReturn1 = exports1['cabi_post_taldawasm:main/mqtt-endpoint#handle-message'];
})();

await $init;
const httpEndpoint = {
  handleRequest: handleRequest,
  
};
const mqttEndpoint = {
  handleMessage: handleMessage,
  
};

export { httpEndpoint, mqttEndpoint, httpEndpoint as 'taldawasm:main/http-endpoint', mqttEndpoint as 'taldawasm:main/mqtt-endpoint' }
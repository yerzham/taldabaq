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

const instantiateCore = WebAssembly.instantiate;

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
let memory0;
let realloc0;
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
  const ret = exports0['taldawasm:main/http-endpoint#handle-request'](ptr1, len1, variant3_0, variant3_1, variant3_2, result7, len7, variant9_0, variant9_1, variant9_2);
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
  const ret = exports0['taldawasm:main/mqtt-endpoint#handle-message'](ptr0, len0);
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
  const module0 = fetchCompile(new URL('./http_endpoint_page_component.core.wasm', import.meta.url));
  ({ exports: exports0 } = await instantiateCore(await module0));
  memory0 = exports0.memory;
  realloc0 = exports0.cabi_realloc;
  postReturn0 = exports0['cabi_post_taldawasm:main/http-endpoint#handle-request'];
  postReturn1 = exports0['cabi_post_taldawasm:main/mqtt-endpoint#handle-message'];
})();

await $init;
const httpEndpoint = {
  handleRequest: handleRequest,
  
};
const mqttEndpoint = {
  handleMessage: handleMessage,
  
};

export { httpEndpoint, mqttEndpoint, httpEndpoint as 'taldawasm:main/http-endpoint', mqttEndpoint as 'taldawasm:main/mqtt-endpoint' }
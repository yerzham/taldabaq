import { httpEndpoint } from './wasm/http_endpoint_proxy_component.js';
const request = {
    path: "/",
    method: {
        tag: 'get',
    },
    headers: [],
    body: new Uint8Array("Hello, world!".split('').map(c => c.charCodeAt(0))),
};
const response = httpEndpoint.handleRequest(request);
const body = response.body;
const headers = response.headers;
console.log(response.statusCode);
for (const header of headers) {
    console.log(`${header[0]}: ${header[1]}`);
}
console.log("============================");
console.log(new TextDecoder().decode(body));

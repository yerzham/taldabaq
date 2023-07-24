import { httpEndpoint as proxy } from '../wasm/http_endpoint_proxy_component.js';
import { httpEndpoint as echo } from '../wasm/http_endpoint_echo_component.js';
import { httpEndpoint as page } from '../wasm/http_endpoint_page_component.js';
import { Headers, Request, Response } from '../wasm/interfaces/http-types.js';
import http from 'node:http';

const sendResponse = (res1: http.ServerResponse, res2: Response) => {
    const headers: Headers = res2.headers;
    headers.forEach(([key, value]) => {
        res1.setHeader(key, value);
    });
    res1.writeHead(res2.statusCode);
    res1.write(res2.body);
    res1.end();
};

const toRequest = (req: http.IncomingMessage): Request => {
    return {
        path: req.url ?? '',
        method: {
            tag: req.method?.toLowerCase() as any,
        },
        headers: Object.entries(req.headers).map(([key, value]) => [key, value?.toString() ?? '']),
        body: new Uint8Array(),
    };
}

const server = http.createServer((req, res) => {
    if (req.url === '/proxy') {
        const response = proxy.handleRequest(toRequest(req));
        sendResponse(res, response);
    } else if (req.url === '/echo') {
        const response = echo.handleRequest(toRequest(req));
        sendResponse(res, response);
    } else if (req.url === '/page') {
        const response = page.handleRequest(toRequest(req));
        sendResponse(res, response);
    } else {
        res.writeHead(404);
        res.end();
    }
});

server.listen(5000, "localhost", () => {
    console.log(`Server is running on http://localhost:5000`);
});
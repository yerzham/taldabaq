import { HttpOutgoing, Request, Response } from "../wasm/interfaces/http-outgoing.js";
import syncFetch from "sync-fetch";
import { Headers } from "../wasm/interfaces/http-types.js";



const fetch_http: typeof HttpOutgoing.fetch = (req: Request): Response => {
    console.log(req.path);
    
    const res = syncFetch(req.path);
    const headers: Headers = [];
    for (const [key, value] of res.headers.entries()) {
        headers.push([key, value]);
    }
    return {
        statusCode: res.status,
        headers: headers,
        body: new Uint8Array(res.arrayBuffer()),
    };
};

export { fetch_http as fetch }
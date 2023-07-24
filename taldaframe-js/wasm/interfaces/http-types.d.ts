export namespace HttpTypes {
}
export type Path = string;
export type Method = MethodGet | MethodHead | MethodPost | MethodPut | MethodDelete | MethodConnect | MethodOptions | MethodTrace | MethodPatch | MethodOther;
export interface MethodGet {
  tag: 'get',
}
export interface MethodHead {
  tag: 'head',
}
export interface MethodPost {
  tag: 'post',
}
export interface MethodPut {
  tag: 'put',
}
export interface MethodDelete {
  tag: 'delete',
}
export interface MethodConnect {
  tag: 'connect',
}
export interface MethodOptions {
  tag: 'options',
}
export interface MethodTrace {
  tag: 'trace',
}
export interface MethodPatch {
  tag: 'patch',
}
export interface MethodOther {
  tag: 'other',
  val: string,
}
export type Headers = [string, string][];
export type Body = Uint8Array;
export interface Request {
  path: Path,
  method: Method,
  headers: Headers,
  body?: Body,
}
export type StatusCode = number;
export interface Response {
  statusCode: StatusCode,
  headers: Headers,
  body?: Body,
}
export type Error = ErrorInvalidUrl | ErrorTimeoutError | ErrorProtocolError | ErrorUnexpectedError;
export interface ErrorInvalidUrl {
  tag: 'invalid-url',
  val: string,
}
export interface ErrorTimeoutError {
  tag: 'timeout-error',
  val: string,
}
export interface ErrorProtocolError {
  tag: 'protocol-error',
  val: string,
}
export interface ErrorUnexpectedError {
  tag: 'unexpected-error',
  val: string,
}

import { CustomResType } from "../packages/types";

// 基础设置
export const baseReturnMap = {
  statusCode: "200,201,202,204,206",
  code: "code",
  data: "data",
  msg: "msg",
  codeValue: 0,
}

// 高级设置
export const responseFuncMap: CustomResType = {
  statusCode: {
    type: 'custom',
    value: `/**
* 根据状态码判断是否请求成功
* @param statusCode: 请求状态码
* @return {boolean}: true 表示请求成功，false 表示请求失败
*/
function response(statusCode){
    return [200, 201, 202, 204, 206].includes(statusCode);
}`
  },
  code: {
    type: 'custom',
    value: `/**
* 根据业务码判断是否请求成功
* @param resData: 响应数据
* @return {boolean}: true 表示请求成功，false 表示请求失败
*/
function response(resData){
    return resData.code === 0;
}`
  },
  data: {
    type: 'custom',
    value: `/**
* 返回请求结果数据
* @param resData: 响应数据
* @return {object} data: 请求结果数据
*/
function response(resData){
    return resData.data;
}`
  },
  msg: {
    type: 'function',
    value: `/**
* 返回请求提示信息
* @param resData: 响应数据
* @return {string} message: 提示信息
*/
function response(resData){
    return resData.msg;
}`
  },
}

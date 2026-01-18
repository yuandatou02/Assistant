import {fetch} from "@tauri-apps/plugin-http";

/**
 * 发起HTTP请求的异步函数
 * @param url - 请求的目标URL地址
 * @param method - HTTP请求方法（如GET、POST等）
 * @param body - 可选的请求体数据，默认为undefined
 * @param timeout - 可选的连接超时时间（毫秒），默认为undefined
 * @returns 返回解析后的JSON数据或null（当响应状态码不是200时）
 */
export const requestFetch = async <T>(url: string, method: string, body?: string, timeout?: number): Promise<T | null> => {
    // 发起fetch请求并等待响应
    const res = await fetch(url, {method, body, connectTimeout: timeout});

    // 检查响应状态码是否为200，如果是则解析JSON数据，否则返回null
    return res.status === 200 ? await res.json() : null;
};


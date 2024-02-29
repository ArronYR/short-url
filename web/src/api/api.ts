import qs from 'querystring';
import {pick} from 'lodash'

export const baseUrl = import.meta.env.VITE_API_HOST ?? '';

const COMMON_HEADERS: Record<string, any> = {
    'Content-Type': 'application/json;charset=utf-8',
    'Api-Secret': '1FIsiEpxQo5l7H',
}

async function search(params?: Request.LinkSearchParam): Promise<Response> {
    return fetch(`${baseUrl}/api/search?${qs.stringify(params)}`, {
        method: 'GET',
        headers: {
            ...COMMON_HEADERS,
        }
    })
}

async function generate(params: Request.LinkAddParam): Promise<Response> {
    return fetch(`${baseUrl}/api/generate`, {
        method: 'POST',
        headers: {
            'Token': params.token,
            ...COMMON_HEADERS,
        },
        body: JSON.stringify({urls: params.urls})
    })
}

async function changeStatus(params: Request.LinkStatusParam): Promise<Response> {
    return fetch(`${baseUrl}/api/status`, {
        method: 'POST',
        headers: {
            'Token': params.token,
            ...COMMON_HEADERS,
        },
        body: JSON.stringify({...pick(params, ['targets', 'status'])})
    })
}

async function changeExpired(params: Request.LinkExpiredParam): Promise<Response> {
    return fetch(`${baseUrl}/api/expired`, {
        method: 'POST',
        headers: {
            'Token': params.token,
            ...COMMON_HEADERS,
        },
        body: JSON.stringify({...pick(params, ['targets', 'expired'])})
    })
}

export default {
    search,
    generate,
    changeStatus,
    changeExpired,
}
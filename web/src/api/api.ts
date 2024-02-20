import qs from 'querystring';

const baseUrl = import.meta.env.VITE_API_HOST ?? ''

async function search(params?: Request.LinkSearchParam): Promise<Response> {
    return fetch(`${baseUrl}/api/search?${qs.stringify(params)}`)
}

async function generate(params: Request.LinkAddParam): Promise<Response> {
    return fetch(`${baseUrl}/api/generate`, {
        method: 'POST',
        headers: {
            'Token': params.token,
            'Content-Type': 'application/json;charset=utf-8'
        },
        body: JSON.stringify({urls: params.urls})
    })
}

export default {
    search,
    generate
}
import {useRequest} from "ahooks";
import {Api} from "../api";
import LinkRowProp = API.LinkRowProp;
import ListResponse = API.ListResponse;
import LinkSearchParam = Request.LinkSearchParam;
import LinkAddParam = Request.LinkAddParam;

const useService = () => {
    const {loading, runAsync: runSearch} = useRequest(Api.search, {
        manual: true,
    });

    const {loading: generating, runAsync: runGenerate} = useRequest(Api.generate, {
        manual: true,
    });

    const search = (params: LinkSearchParam): Promise<ListResponse<LinkRowProp>> => {
        return new Promise((resolve, reject) => {
            runSearch(params).then(response => {
                if (response.status !== 200) {
                    reject(`请求失败：${response.status}`)
                }
                response.json()
                    .then((res) => {
                        resolve(res as ListResponse<LinkRowProp>);
                    })
                    .catch((err) => {
                        reject(err.toString());
                    })
            }).catch((err) => {
                reject(err.toString());
            })
        })
    }

    const generate = (params: LinkAddParam) => {
        return new Promise((resolve, reject) => {
            runGenerate(params).then(async (response) => {
                if (response.status !== 200) {
                    const text = await response.text();
                    reject(`${response.status} - ${text}`)
                }
                response.json()
                    .then((res) => {
                        resolve(res as ListResponse<LinkRowProp>)
                    })
                    .catch((err) => {
                        reject(err.toString())
                    })
            }).catch((err) => {
                reject(err.toString());
            })
        })
    }

    return {
        search,
        loading,
        generate,
        generating,
    }
}

export default useService

import {useRequest} from "ahooks";
import {Api} from "../api";

const useService = () => {
    const {loading, runAsync: runSearch} = useRequest(Api.search, {
        manual: true,
    });

    const {loading: generating, runAsync: runGenerate} = useRequest(Api.generate, {
        manual: true,
    });

    const {loading: statusChanging, runAsync: runChangeStatus} = useRequest(Api.changeStatus, {
        manual: true,
    });

    const {loading: expiredChanging, runAsync: runChangeExpired} = useRequest(Api.changeExpired, {
        manual: true,
    });

    const search = (params: Request.LinkSearchParam): Promise<API.ListResponse<API.LinkRowProp>> => {
        return new Promise((resolve, reject) => {
            runSearch(params).then(response => {
                if (response.status !== 200) {
                    reject(`请求失败：${response.status}`)
                }
                response.json()
                    .then((res) => {
                        resolve(res as API.ListResponse<API.LinkRowProp>);
                    })
                    .catch((err) => {
                        reject(err.toString());
                    })
            }).catch((err) => {
                reject(err.toString());
            })
        })
    }

    const generate = (params: Request.LinkAddParam) => {
        return new Promise((resolve, reject) => {
            runGenerate(params).then(async (response) => {
                if (response.status !== 200) {
                    const text = await response.text();
                    reject(`${response.status} - ${text}`)
                }
                response.json()
                    .then((res) => {
                        resolve(res as API.ListResponse<API.LinkRowProp>)
                    })
                    .catch((err) => {
                        reject(err.toString())
                    })
            }).catch((err) => {
                reject(err.toString());
            })
        })
    }

    const changeStatus = (params: Request.LinkStatusParam) => {
        return new Promise((resolve, reject) => {
            runChangeStatus(params).then(async (response) => {
                if (response.status !== 200) {
                    const text = await response.text();
                    reject(`${response.status} - ${text}`)
                }
                response.json()
                    .then((res) => {
                        resolve(res)
                    })
                    .catch((err) => {
                        reject(err.toString())
                    })
            }).catch((err) => {
                reject(err.toString());
            })
        })
    }

    const changeExpired = (params: Request.LinkExpiredParam) => {
        return new Promise((resolve, reject) => {
            runChangeExpired(params).then(async (response) => {
                if (response.status !== 200) {
                    const text = await response.text();
                    reject(`${response.status} - ${text}`)
                }
                response.json()
                    .then((res) => {
                        resolve(res)
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
        changeStatus,
        statusChanging,
        changeExpired,
        expiredChanging,
    }
}

export default useService

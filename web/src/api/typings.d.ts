declare namespace API {
    export type ListResponse<T> = {
        pages?: number;
        size?: number;
        records?: T[];
    }
    export type LinkRowProp = {
        id?: number;
        shortId?: string,
        originalUrl?: string;
        status?: number;
        expiredTs?: number;
        pv?: number;
    }
}

declare namespace Request {
    export type LinkSearchParam = {
        page?: number;
        size?: number;
        keyword?: string;
    }

    export type LinkAddParam = {
        token: string;
        urls: string[];
    }

    export type LinkStatusParam = {
        token: string;
        targets: string[];
        status: number;
    }

    export type LinkExpiredParam = {
        token: string;
        targets: string[];
        expired: number;
    }
}

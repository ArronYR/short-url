declare namespace API {
    export type ListResponse<T> = {
        pages?: number;
        size?: number;
        records?: T[];
    }
    export type LinkRowProp = {
        id?: number;
        short_id?: string,
        original_url?: string;
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
}
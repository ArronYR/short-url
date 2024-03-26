export const LINK_STATUS = {
    UNKNOWN: -1,
    NORMAL: 0,
    DISABLED: 1,
}

export const LINK_STATUS_TEXT = {
    [LINK_STATUS.UNKNOWN]: '未知',
    [LINK_STATUS.NORMAL]: '启用',
    [LINK_STATUS.DISABLED]: '已禁用',
}

export const LINK_STATUS_COLOR: Record<number, 'default' | 'primary' | 'secondary' | 'error' | 'info' | 'success' | 'warning'> = {
    [LINK_STATUS.UNKNOWN]: 'default',
    [LINK_STATUS.NORMAL]: 'success',
    [LINK_STATUS.DISABLED]: 'warning',
}

export const LINK_STATUS_BUTTON_TEXT = {
    [LINK_STATUS.NORMAL]: '启用',
    [LINK_STATUS.DISABLED]: '禁用',
}

export const LINK_STATUS_BUTTON_COLOR: Record<number, 'inherit' | 'primary' | 'secondary' | 'success' | 'error' | 'info' | 'warning'> = {
    [LINK_STATUS.UNKNOWN]: 'error',
    [LINK_STATUS.NORMAL]: 'error',
    [LINK_STATUS.DISABLED]: 'success',
}

export const DT_FORMAT = {
    DATE: 'YYYY-MM-DD',
    TIME: 'HH:mm:ss',
    DATETIME: 'YYYY-MM-DD HH:mm:ss'
}

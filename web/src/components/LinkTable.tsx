import * as React from 'react';
import {DataGrid, GridColDef} from '@mui/x-data-grid';
import Link from "@mui/material/Link";
import LinkRowProp = API.LinkRowProp;
import {useSafeState} from "ahooks";
import {useEffect} from "react";
import useService from "../service/service";
import {Button, TextField, Tooltip, Typography} from '@mui/material';
import SnackAlert from "./SnackAlert";
import {Add} from "@mui/icons-material";
import AddFormDialog from "./AddFormDialog";


export default function LinkTable() {
    const [links, setLinks] = useSafeState<LinkRowProp[]>([]);
    const [pageSize] = useSafeState<number>(30);
    const [total, setTotal] = useSafeState<number>(0);
    const [keyword, setKeyword] = useSafeState<string>();
    const [alertVisible, setAlertVisible] = useSafeState<boolean>(false);
    const [alertMessage, setAlertMessage] = useSafeState<string>('');
    const [addVisible, setAddVisible] = useSafeState<boolean>(false);

    const {loading, search} = useService();
    const onSearch = (page?: number, keyword?: string) => {
        search({
            page: page ?? 1,
            size: pageSize,
            keyword: keyword?.trim() || undefined
        }).then((res) => {
            setLinks(res.records ?? [])
            setTotal((res.size ?? 0) * (res.pages ?? 1))
        })
    }

    useEffect(() => {
        onSearch(1)
    }, [])

    const showAlert = (message: string) => {
        setAlertVisible(true)
        setAlertMessage(message)
    }

    const columns: GridColDef<LinkRowProp>[] = [
        {
            field: 'short_id',
            headerName: '短链接',
            width: 300,
            sortable: false,
            renderCell: (props) => {
                return (
                    <Typography
                        variant="body2"
                        color="text.secondary"
                        align="center"
                        display={"flex"}
                        justifyContent={"space-between"}
                        width={'90%'}
                    >
                        {`${window.location.origin}/${props.row.short_id}`}
                        <Link color="inherit" href={`/${props.row.short_id}`} target={"_blank"}>访问</Link>
                    </Typography>
                )
            }
        },
        {
            field: 'original_url',
            headerName: '原链接',
            minWidth: 600,
            cellClassName: 'cell-cls-name',
            sortable: false,
            renderCell: (props) => {
                return (
                    <Typography
                        variant="body1"
                        variantMapping={{body1: 'div'}}
                        color="text.secondary"
                        align="center"
                        display={"flex"}
                        justifyContent={"space-between"}
                        width={'90%'}
                    >
                        <Tooltip title={props.row.original_url} arrow={true} placement={"top"}>
                            <Typography
                                variant="body2"
                                noWrap={true}
                                textOverflow={"ellipsis"}
                            >{props.row.original_url}</Typography>
                        </Tooltip>
                        <Link sx={{ml: 4}} color="inherit" href={props.row.original_url} target={"_blank"}>访问</Link>
                    </Typography>
                )
            }
        }
    ];

    return (
        <div style={{width: '100%'}}>
            <div style={{marginBottom: 6, display: "flex", alignItems: 'center', justifyContent: 'space-between'}}>
                <div style={{marginBottom: 6, display: "flex", alignItems: 'end'}}>
                    <TextField
                        label={"请输入关键词"}
                        variant={"standard"}
                        inputMode={'text'}
                        onChange={(event) => {
                            setKeyword(event.target.value)
                        }}
                    />
                    <Button
                        variant="text"
                        size={'small'}
                        onClick={() => {
                            onSearch(1, keyword)
                        }}
                    >查询</Button>
                </div>
                <Button
                    variant="outlined"
                    startIcon={<Add/>}
                    onClick={() => {
                        setAddVisible(true);
                    }}
                >添加</Button>
            </div>

            <div style={{height: 600, width: "100%"}}>
                <DataGrid
                    loading={loading}
                    rows={links}
                    columns={columns}
                    rowCount={total}
                    rowHeight={42}
                    rowSelection={false}
                    disableRowSelectionOnClick={true}
                    initialState={{
                        pagination: {
                            paginationModel: {page: 0, pageSize},
                        },
                    }}
                    pageSizeOptions={[pageSize]}
                    paginationMode={'server'}
                    onPaginationModelChange={(model) => {
                        onSearch(model.page + 1, keyword)
                    }}
                />
            </div>
            <AddFormDialog
                visible={addVisible}
                onCancel={() => {
                    setAddVisible(false);
                }}
                onOk={() => {
                    setAddVisible(false);
                    onSearch(1, keyword)
                }}
            />
            <SnackAlert
                visible={alertVisible}
                message={alertMessage}
                color={'warning'}
                onClose={() => {
                    setAlertVisible(false)
                    setAlertMessage('')
                }}
            />
        </div>
    )
}
import * as React from 'react';
import {
    Button,
    Dialog,
    DialogActions,
    DialogContent,
    DialogContentText,
    DialogTitle,
    List,
    ListItem,
    ListItemText,
    TextField
} from '@mui/material';
import SnackAlert from "./SnackAlert";
import {LINK_STATUS, LINK_STATUS_BUTTON_TEXT} from "../config/constants";
import {baseUrl} from "../api/api";
import {useMemo} from "react";
import useService from "../service/service";
import {isNil, trim} from "lodash";
import {useAlert} from "../hooks";

type Props = {
    visible: boolean;
    targets: string[];
    status?: number;
    onOk?: () => void;
    onCancel?: () => void;
}

export default function ChangeStatusDialog(props: Props) {
    const {visible, targets, status, onOk, onCancel} = props;

    const {alertVisible, alertMessage, alertColor, showAlert, closeAlert} = useAlert();
    const {changeStatus, statusChanging} = useService();

    const statusText = useMemo(() => {
        return status === LINK_STATUS.NORMAL
            ? LINK_STATUS_BUTTON_TEXT[LINK_STATUS.NORMAL]
            : LINK_STATUS_BUTTON_TEXT[LINK_STATUS.DISABLED]
    }, [status])

    const handleDialogClose = (_event: {}, reason: 'backdropClick' | 'escapeKeyDown') => {
        if (reason === 'backdropClick') {
            return
        }
        handleClose()
    }

    const handleClose = () => {
        if (onCancel) {
            onCancel();
        }
    }

    const handleSubmit = (data: Record<string, string>) => {
        const {token} = data;
        if (!targets?.length) {
            showAlert('请点击要启用的短链接', 'error');
            return
        }
        if (isNil(status)) {
            showAlert('当前数据有误，请刷新或联系管理员', 'error');
            return;
        }
        if (!trim(token).length) {
            showAlert('请填写正确的安全码', "error");
            return;
        }
        changeStatus({
            token,
            targets,
            status: status,
        }).then(() => {
            showAlert('操作成功', 'success');
            if (onOk) {
                onOk();
            }
            handleClose();
        }).catch((err) => {
            showAlert(err.toString(), 'error');
        })
    }

    return (
        <div className={'dialog-container'}>
            <Dialog
                open={visible}
                disableEscapeKeyDown={true}
                PaperProps={{
                    component: 'form',
                    onSubmit: (event: React.FormEvent<HTMLFormElement>) => {
                        event.preventDefault();
                        const formData = new FormData(event.currentTarget);
                        const formJson = Object.fromEntries((formData as any).entries());
                        handleSubmit(formJson);
                    },
                }}
                onClose={handleDialogClose}
            >
                <DialogTitle>{statusText}短链接</DialogTitle>
                <DialogContent>
                    <DialogContentText>
                        您将对以下短链接进行 <strong>{statusText}</strong> 操作，请慎重。
                    </DialogContentText>
                    <List disablePadding={true}>
                        {(targets || []).map((value) => {
                            return (
                                <ListItem key={value} disablePadding={true}>
                                    <ListItemText
                                        primary={`${baseUrl}/${value}`}
                                    />
                                </ListItem>
                            )
                        })}
                    </List>
                    <TextField
                        name={'token'}
                        type={'text'}
                        label={'安全码'}
                        margin={'dense'}
                        variant={'standard'}
                        autoFocus={true}
                        required={true}
                        placeholder={'请输入安全码'}
                    />
                </DialogContent>
                <DialogActions>
                    <Button onClick={handleClose}>取消</Button>
                    <Button
                        type={'submit'}
                        disabled={statusChanging}
                    >确定</Button>
                </DialogActions>
            </Dialog>
            <SnackAlert
                visible={alertVisible}
                message={alertMessage}
                color={alertColor}
                onClose={closeAlert}
            />
        </div>
    )
}
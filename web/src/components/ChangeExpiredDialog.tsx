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
import {baseUrl} from "../api/api";
import useService from "../service/service";
import {trim} from "lodash";
import {useAlert} from "../hooks";
import {DateTimeField, LocalizationProvider} from "@mui/x-date-pickers";
import {AdapterDayjs} from "@mui/x-date-pickers/AdapterDayjs";
import {DT_FORMAT} from "../config/constants";
import moment from "moment";

type Props = {
    visible: boolean;
    targets: string[];
    defaultValue?: number;
    onOk?: () => void;
    onCancel?: () => void;
}

export default function ChangeExpiredDialog(props: Props) {
    const {visible, targets, defaultValue, onOk, onCancel} = props;

    const {alertVisible, alertMessage, alertColor, showAlert, closeAlert} = useAlert();
    const {changeExpired, expiredChanging} = useService();

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
        const {token, datetime} = data;
        if (!targets?.length) {
            showAlert('请点击要启用的短链接', 'error');
            return
        }
        let expired = 0;
        if (datetime) {
            if (!moment(datetime).isValid()) {
                showAlert('请正确设置有效期的格式', 'error');
                return;
            }
            expired = moment(datetime).valueOf();
        }
        if (!trim(token).length) {
            showAlert('请填写正确的安全码', "error");
            return;
        }
        changeExpired({
            token,
            targets,
            expired,
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
                <DialogTitle>设置有效期</DialogTitle>
                <DialogContent>
                    <DialogContentText>
                        您将对以下短链接设置有效期，请慎重操作。
                    </DialogContentText>
                    <List disablePadding={true} sx={{mb: 2}}>
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
                    <LocalizationProvider dateAdapter={AdapterDayjs}>
                        <DateTimeField
                            name={'datetime'}
                            label={'有效期'}
                            ampm={false}
                            format={DT_FORMAT.DATETIME}
                            clearable={true}
                            size={'small'}
                            fullWidth={true}
                            value={defaultValue ? moment(defaultValue) : null}
                        />
                    </LocalizationProvider>
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
                        disabled={expiredChanging}
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
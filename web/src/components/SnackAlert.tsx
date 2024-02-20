import * as React from 'react';
import {Alert, Snackbar} from '@mui/material';
import {AlertColor} from "@mui/material/Alert/Alert";

type Props = {
    visible: boolean;
    message: string | JSX.Element;
    duration?: number;
    color?: AlertColor;
    onClose?: () => void;
}

export default function SnackAlert(props: Props) {
    const {visible, message, duration, color, onClose} = props;

    const handleClose = (event?: React.SyntheticEvent | Event, reason?: string) => {
        if (reason === 'clickaway') {
            return;
        }
        if (onClose) {
            onClose();
        }
    };

    return (
        <Snackbar
            open={visible}
            autoHideDuration={duration ?? 2_000}
            anchorOrigin={{vertical: 'top', horizontal: 'center'}}
            onClose={handleClose}
        >
            <Alert
                onClose={handleClose}
                severity={color}
                variant={"filled"}
                sx={{width: '100%'}}
            >
                {message}
            </Alert>
        </Snackbar>
    );
}
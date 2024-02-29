import {useSafeState} from "ahooks";
import {AlertColor} from "@mui/material/Alert/Alert";

const useAlert = () => {
    const [alertVisible, setAlertVisible] = useSafeState<boolean>(false);
    const [alertMessage, setAlertMessage] = useSafeState<string>('');
    const [alertColor, setAlertColor] = useSafeState<AlertColor>('success');

    const showAlert = (message: string, color?: AlertColor) => {
        setAlertVisible(true);
        setAlertMessage(message);
        if (color) {
            setAlertColor(color);
        }
    }

    const closeAlert = () => {
        setAlertVisible(false);
        setAlertMessage('');
    }

    return {
        alertVisible,
        alertMessage,
        alertColor,
        showAlert,
        closeAlert,
    }
}

export default useAlert;

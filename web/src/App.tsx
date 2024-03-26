import * as React from 'react';
import Container from '@mui/material/Container';
import Typography from '@mui/material/Typography';
import Box from '@mui/material/Box';
import Link from '@mui/material/Link';
import ProTip from './components/ProTip';
import LinkTable from "./components/LinkTable";

function Copyright() {
    return (
        <Typography sx={{mt: 4}} variant={"body2"} color={"text.secondary"} align={"center"}>
            {'Copyright © '}
            {new Date().getFullYear()}
        </Typography>
    );
}

export default function App() {
    return (
        <Container>
            <Box sx={{my: 4}}>
                <Typography variant={"h4"} component={"h1"} sx={{mb: 2}}>短链接</Typography>
                <ProTip/>
                <LinkTable/>
                <Copyright/>
            </Box>
        </Container>
    );
}

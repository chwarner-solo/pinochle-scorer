import {Card, CardContent, Typography} from "@mui/material";

export const TitleCard = (props) => (
    <Card size={{width: { xs: '100%', sm: 500, md: 600 } }} style={{ maxWidth: 600, marginBottom: 16, width: '100%'}}>
        <CardContent>
            <Typography
                variant="h3"
                align="center"
                gutterBottom
                style={{ fontWeight: 700, letterSpacing: 2}}
            >
                Pinochle Score Keeper
            </Typography>
        </CardContent>
    </Card>
)
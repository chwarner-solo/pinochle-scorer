import { Card, CardContent, Grid, Paper, Typography } from "@mui/material";

export const StatsCard = (props) => {
    const { usScore, themScore, trump, requiredTricks } = props;

    return (
        <Card size={{ width: { xs: '100%', sm: 500, md: 600 } }} style={{ maxWidth: 600, marginBottom: 16, width: '100%'}}>
            <CardContent>
                <Grid container spacing={2} alignItems="center" justifyContent="space-between">
                    <Grid size={{ xs: 4}}>
                        <Paper style={{ padding: 16, textAlign: 'center'}} elevation={2}>
                            <Typography variant="subtitle1">US Score</Typography>
                            <Typography variant="h5">{usScore}</Typography>
                        </Paper>
                    </Grid>
                    <Grid size={{ xs: 4}}>
                        <Paper style={{ padding: 16, textAlign: 'center'}} elevation={2}>
                            <Typography variant="subtitle1">Trump</Typography>
                            <Typography variant="h5">{trump}</Typography>
                            <Typography variant="body2" style={{ marginTop: 8 }}>
                                Tricks to Save: <b>{requiredTricks}</b>
                            </Typography>
                        </Paper>
                    </Grid>
                    <Grid size={{ xs: 4}}>
                        <Paper style={{ padding: 16, textAlign: 'center'}} elevation={2}>
                            <Typography variant="subtitle1">THEM Score</Typography>
                            <Typography variant="h5">{themScore}</Typography>
                        </Paper>
                    </Grid>
                </Grid>
            </CardContent>
        </Card>
    )
}
import {Alert, Box, Button, CircularProgress, Container, Typography} from "@mui/material";
import {useGame} from "../hooks/useGame.ts";

export default () => {
    const { game, loading, error, createGame, startHand } = useGame();

    return (
        <Container>
            <Typography variant="h3" align="center" sx={{my: 4}}>
                Pinochle Scorer
            </Typography>

            {error && (
                <Alert severity="error" sx={{mb: 2}}>
                    {error}
                </Alert>
            )}

            <Box textAlign="center">
                {!game ? (
                    <Button
                        variant="contained"
                        size="large"
                        onClick={createGame}
                        disabled={loading}>
                        {loading ? <CircularProgress size={24} /> : 'Start New Game'}
                    </Button>
                ) : (
                    <Box>
                        <Typography variant="h6" sx={{ mb: 2}}>
                            Game Created! State: {game.state}
                        </Typography>
                        {game.state === 'WaitingToStart' && (
                            <Button
                                variant="contained"
                                onClick={startHand}
                                disabled={loading}
                            >
                                {loading ? <CircularProgress size={24} /> : 'Start New Hand'}
                            </Button>
                        )}
                    </Box>
                )}
            </Box>
        </Container>
    );
}
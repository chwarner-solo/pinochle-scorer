import {useNavigate} from "react-router-dom";
import {useGame} from "../hooks/useGame.ts";
import {Container, Typography, Alert, Box, Card, CardContent, CircularProgress, Button} from "@mui/material";

const GamePage = () => {
    const navigate = useNavigate();
    const { game, loading, error, createGame} = useGame();

    const handleCreateGame = async () => {
        await createGame();
        if (game) {
            navigate(`/games/${game.id()}`);
        }
    };

    return (
        <Container maxWidth="sm">
            <Typography variant="h3" align="center" sx={{my: 4}}>
                Pinochle Scorer
            </Typography>
            {error && (
                <Alert severity="error" sx={{mb: 2}}>
                    {error}
                </Alert>
            )}

            <Card>
                <CardContent>
                    <Box textAlign="center">
                        <Typography variant="h5" gutterBottom>
                            Start New Game
                        </Typography>
                        <Button
                            variant="contained"
                            size="large"
                            onClick={handleCreateGame}
                            disabled={loading}
                        >
                            {loading ? <CircularProgress size={24} /> : 'Start New Game'}
                        </Button>
                    </Box>
                </CardContent>
            </Card>
        </Container>
    );
}

export default GamePage;
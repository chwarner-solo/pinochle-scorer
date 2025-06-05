import {useNavigate} from "react-router-dom";
import {useGame} from "../hooks/useGame.ts";
import {Container, Typography, Alert, Card, CardContent, CircularProgress, Button} from "@mui/material";
import {useEffect} from "react";

const ErrorAlert = ({ error}: {error? : string}) => error ? <Alert severity="error" sx={{mb: 2}}>{error}</Alert> : null;

const StartGameCard = ({ onStart, loading }: { onStart: () => void, loading: boolean }) => (
    <Card>
        <CardContent>
            <Button onClick={onStart} disabled={loading}>
                {loading ? <CircularProgress size={24} /> : 'Start New Game'}
            </Button>
        </CardContent>
    </Card>
);

const GamePage = () => {
    const navigate = useNavigate();
    const { game, loading, error, createGame} = useGame();

    useEffect(() => {
        if (game && game.game_id) {
            navigate(`/games/${game.game_id}`);
        }
    }, [game, navigate]);

    return (
        <Container maxWidth="sm">
            <Typography variant="h3">Pinochle Scorer</Typography>
            <ErrorAlert error={error} />
            <StartGameCard onStart={createGame} loading={loading} />
        </Container>
    );
}

export default GamePage;
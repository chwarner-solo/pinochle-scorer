import {Box, Container, Stack, Typography} from "@mui/material";
import {useGame} from "../hooks/useGame.ts";
import {realGameApi} from "../services/api.ts";
import {TitleCard} from "../components/game/TitleCard.tsx";
import {StatsCard} from "../components/game/StatsCard.tsx";
import FormCard from "../components/game/FormCard.tsx";
import CompletedHandsCard from "../components/game/CompletedHandsCard.tsx";

export default () => {
    const gameState = useGame(realGameApi);
    return (
        <Stack spacing={2} alignItems="center" width="100%" mt={2}>
            <TitleCard {...gameState} />
            <StatsCard {...gameState} />
            <FormCard {...gameState} />
            <CompletedHandsCard {...gameState} />
        </Stack>
    );
}
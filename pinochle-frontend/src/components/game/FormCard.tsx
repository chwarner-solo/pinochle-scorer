import {Button, Card, CardContent, Typography} from "@mui/material";
import type {GameState, HandState} from "../../types/Game.ts";

const StartGameForm = ({onSubmit}) => (
    <Button variant="contained" onClick={onSubmit} fullWidth>
        Start New Game
    </Button>
);
const StartHandForm = ({onSubmit}) => (
    <Button variant="contained" onClick={onSubmit} fullWidth>
        Start New Hand
    </Button>);
const BidForm = () => (
    <Typography variant="h6">Bid Form</Typography>
);
const TrumpForm = () => (
    <Typography variant="h6">Trump Form</Typography>
);
const MeldForm = () => (
    <Typography variant="h6">Meld Form</Typography>
);
const TricksForm = () => (
    <Typography variant="h6">Tricks Form</Typography>
);

type GameFormMap = {
    [K in GameState]: React.FC<{ onSubmit: () => void }> | null
}

const gameFormMap  : GameFormMap = {
    NoGame: StartGameForm,
    WaitingToStart: StartHandForm,
    InProgress: null,
    Completed: () => (
        <Typography align="center" variant="h6">Game Completed</Typography>
    ),
};

type HandFormMap = {
    [K in HandState]: React.FC<{ onSubmit: () => void }> | null
}

const handFormMap : HandFormMap = {
    WaitingForBid: BidForm,
    WaitingForTrump: TrumpForm,
    WaitingForMeld: MeldForm,
    WaitingForTricks: TricksForm,
    Completed: () => (
        <Typography align="center" variant="h6">Hand Completed</Typography>
    ),
    NoHand: null,
    NoMarriage: MeldForm
};

const FormCard = (props) => {
    const { state, handState, onSubmit } = props;

    let FormComponent = gameFormMap[state];
    if (state === 'InProgress' && handState) {
        FormComponent = handFormMap[handState];
    }

    return (
        <Card size={{ width: { xs: '100%', sm: 500, md: 600 }}} style={{ maxWidth: 600, marginBottom: 16, width: '100%'}}>
            <CardContent>
                {FormComponent ? (
                    <FormComponent onSubmit={onSubmit} />
                ) : (<></>)}
            </CardContent>
        </Card>
    )
}

export default FormCard;
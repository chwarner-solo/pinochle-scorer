import React from "react";
import {Button, Typography, Box} from "@mui/material";
import {Hand} from "../types/Game.ts";
import { TrumpFormData } from "../types/Hand.ts";

type Props = {
    hand: Hand;
    onSubmit: (form: { state: "WaitingForTrump"; data: TrumpFormData}) => void;
    isSubmitting: boolean;
};

export const TrumpFormComponent = React.FC<Props> = ({ hand, onSubmit, isSubmitting }) => (
    <Box>
        <Typography variant="h6">Trump Phase</Typography>
        <Button
            variant="contained"
            color="primary"
            disabled={isSubmitting}
            onClick={() => onSubmit({ state: "WaitingForTrump", data: {trump: hand.trump}})}
        >
            Submit Trump
        </Button>
    </Box>
);
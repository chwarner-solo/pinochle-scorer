import React from "react";
import { Box, Button, Typography } from '@mui/material';
import { Hand } from "../types/Game";
import { BidFormData } from "../types/form_types";

type Props = {
    hand: Hand;
    onSubmit: (form: { state: "WaitingForBid"; data: BidFormData }) => void;
    isSubmitting: boolean;
};

export const BidFormComponent = React.FC<Props> = ({ hand, onSubmit, isSubmitting }) => (
    <Box>
        <Typography variant="h6">Bid Phase</Typography>
        <Button
            variant="contained"
            color="primary"
            disabled={isSubmitting}
            onClick={() => onSubmit({ state: "waitingForBid", data: { bid: 0, player: ""}})}
        >
            Submit Stub Bid
        </Button>
    </Box>
)
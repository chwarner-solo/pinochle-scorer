import {
    Box, Button,
    Card,
    CardContent,
    Container,
    FormControl,
    Grid,
    InputLabel,
    MenuItem,
    Select, TextField,
    Typography
} from "@mui/material";
import type {Player} from "../types/Game.ts";
import React, {useEffect, useState} from "react";

const bidderOptions = [
    { value: "North", label: "↑" },
    { value: "South", label: "↓" },
    { value: "West",  label: "←" },
    { value: "East",  label: "→" }
];

const validateBid = (amount: number): boolean =>
    amount >= 50 &&
    (
        amount <= 59 ||
        (amount >= 60 && amount <= 99 && amount % 5 === 0) ||
        (amount >= 100 && amount % 10 === 0)
    );

const ScoreHeader = ({usTotal, themTotal, trump, requiredTricks} : {usTotal: number, themTotal: number, trump: string, requiredTricks: number}) => (
    <Card sx={{ mb: 3}}>
        <CardContent>
            <Grid container alignItems="center" justifyContent="space-between">
                <Grid item xs={4}>
                    <Box textAlign="left">
                        <Typography variant="h3" align="center" color="Primary">{usTotal}</Typography>
                        <Typography variant="h6" align="center" color="Primary">US</Typography>
                    </Box>
                </Grid>
                <Grid item xs={6}>
                    <Box textAlign="left">
                        <Typography variant="h6">Trump: {trump || 'Not Set'}</Typography>
                        <Typography variant="body1">Required Tricks: {requiredTricks}</Typography>
                    </Box>
                </Grid>
                <Grid item xs={4}>
                    <Box textAlign="right">
                        <Typography variant="h3" align="center" color="error">{themTotal}</Typography>
                        <Typography variant="h6" align="center" color="error">THEM</Typography>
                    </Box>
                </Grid>
            </Grid>
        </CardContent>
    </Card>
);

const BidderSelector = ({bidder, setBidder}: {bidder: Player | '', setBidder: (b: Player) => void}) => {
    useEffect(() => {
        const handleKeydown = (e: KeyboardEvent) => {
            const key = e.key.toUpperCase();
            if (['S', 'W', 'N', 'E'].includes(key)) {
                const map: Record<string, Player> = {S: "South", W: "West", N: "North", E: "East"};
                setBidder(map[key]);
            }
        };
        window.addEventListener('keydown', handleKeydown);
        return () => window.removeEventListener('keydown', handleKeydown);
    }, [setBidder]);

    return (
        <Box display="flex" flexDirection="row" alignItems="center" gap={1}>
            {bidderOptions.map(opt => (
                <Button
                    key={opt.value}
                    variant={bidder === opt.value ? "contained" : "outlined"}
                    onClick={() => setBidder(opt.value as Player)}
                >
                    {opt.label}
                </Button>
            ))}
        </Box>
    );
}

const BidInputChange = (
    {bidder, setBidder, bidAmount, setBidAmount, bidError, onSubmit, isValidBid
}:{
    bidder: Player | '',
    setBidder: (bidder: Player | '') => void,
    bidAmount: string,
    setBidAmount: (amount: string) => void,
    bidError: string,
    onSubmit: () => void,
    isValidBid: boolean
}) => (
    <Card>
        <CardContent>
            <Typography variant="h6" gutterBottom>Record Bid</Typography>
            <Grid container spacing={2} alignItems="center">
                <Grid item xs={3}>
                    <Box mb={2}>
                        <BidderSelector bidder={bidder} setBidder={setBidder} />
                    </Box>
                </Grid>
            </Grid>
            <Grid item xs={12} md={6}>
                <TextField
                    fullWidth
                    label="Bid Amount"
                    type="number"
                    value={bidAmount}
                    onChange={(e) => setBidAmount(e.target.value)}
                    error={!!bidError}
                    helperText={bidError || 'Min 50, then by 1 till 60, by 5 till 100, then by 10'}
                />
            </Grid>
            <Grid item xs={12}>
                <Button
                    variant="contained"
                    fullWidth
                    onClick={onSubmit}
                    disabled={!isValidBid}
                >
                    Record Bid
                </Button>
            </Grid>
        </CardContent>
    </Card>
);

function getBidError(amount: number, setBidError: (e: string) => void): string {
    if (isNaN(amount)) return 'Please enter a number';
    if (amount < 50) return 'Bid must be at least 50';
    if (amount >= 60 && amount <= 99 && amount % 5 !== 0) return 'Bid must be a multiple of 5 when between 60 and 99';
    if (amount >= 100 && amount % 10 !== 0)
        return 'Bids 100+ must be multiples of 10';
    return '';
}

const handleBidAmountChange = (value: string, setBidAmount: (v: string) => void, setBidError: (e: string) => void) => {
    setBidAmount(value);
    const amount = parseInt(value);
    const error = getBidError(amount, setBidError);
};

const handleSubmitBid = () => {
    const amount = parseInt(bidAmount);

    console.log('Recording bid::', {bidder, amount});
}

const HandPage = () => {
    const [bidder, setBidder] = useState<Player | ''>('');
    const [bidAmount, setBidAmount] = useState<string>('');
    const [bidError, setBidError] = useState<string>('');

    const usTotal = 0, themTotal = 0, trump = '', requiredTricks = 0;


    const isValidBid = bidder && bidAmount && !bidError && validateBid(parseInt(bidAmount));

    return (
        <Container maxWidth="md">
            <Typography variant="h4" sx={{my: 3}}>
                Current Hand
            </Typography>
            <ScoreHeader usTotal={usTotal} themTotal={themTotal} trump={trump} requiredTricks={requiredTricks} />
            <BidInputChange
                bidder={bidder}
                setBidder={setBidder}
                bidAmount={bidAmount}
                setBidAmount={v => handleBidAmountChange(v, setBidAmount, setBidError)}
                bidError={bidError}
                setBidError={setBidError}
                onSubmit={() =>handleSubmitBid(bidder, bidAmount)}
                isValidBid={!!isValidBid}
            />
        </Container>
    )
}

export default HandPage
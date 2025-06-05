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
import { Player } from "../types/Game.ts"
import React, {useState} from "react";

const validateBid = (amount: number): boolean => {
    if (amount < 50) return false;
    if (amount <= 59) return true;
    if (amount <= 99) return amount % 5 === 0;
    return amount % 10 === 0
}
const HandPage = () => {
    const [bidder, setBidder] = useState<Player | ''>('');
    const [bidAmount, setBidAmount] = useState<string>('');
    const [bidError, setBidError] = useState<string>('');

    const usTotal = 0;
    const themTotal = 0;
    const trump = '';
    const requiredTricks = 0;

    const handleBidAmountChange = (value: string) => {
        setBidAmount(value);
        const amount = parseInt(value);

        if (!isNaN(amount)) {
            setBidError('');
            return;
        }

        if (!validateBid(amount)) {
            if (amount < 50) {
                setBidError('Bid must be at least 50');
            } else if (amount >= 60 && amount <= 99 && amount % 5 !== 0) {
                setBidError('Bid must be a multiple of 5 when between 60 and 99');
            } else if (amount >= 100 && amount % 10 !== 0) {
                setBidError('Bids 100+ must be multiples of 10');
            }
        } else {
            setBidError('');
        }
    };

    const handleSubmitBid = () => {
        const amount = parseInt(bidAmount);

        console.log('Recording bid::', {bidder, amount});
    }

    const isValidBid = bidder && bidAmount && !bidError && validateBid(parseInt(bidAmount));

    return (
        <Container maxWidth="md">
            <Typography vartiant="h4" sx={{my: 3}}>
                Current Hand
            </Typography>
            {/* Score Header */}
            <Card sx={{ mb: 3}}>
                <CardContent>
                    <Grid container spacing={2} alignItems="center">
                        <Grid item xs={3}>
                            <Typography variant="h3" align="center" color="primary">
                                {usTotal}
                            </Typography>
                            <Typography variant="h6" align="center" color="primary">
                                US
                            </Typography>
                        </Grid>
                        <Grid item xs={6}>
                            <Box textAlign="center">
                                <Typography variant="h6">
                                    Trump: {trump || 'Not Set'}
                                </Typography>
                                <Typography variant="body1">
                                    Required Tricks: {requiredTricks}
                                </Typography>
                            </Box>
                        </Grid>
                        <Grid item xs={3}>
                            <Typography variant="h3" align="center" color="error">
                                {themTotal}
                            </Typography>
                            <Typography variant="h6" align="center" color="error">
                                THEM
                            </Typography>
                        </Grid>
                    </Grid>
                </CardContent>
            </Card>
            {/* Bid Input */}
            <Card>
                <CardContent>
                    <Typography variant="h6" gutterBottom>
                        Record Bid
                    </Typography>

                    <Grid container spacing={2}>
                        <Grid item xs={12} md={6}>
                            <FormControl fullWidth>
                                <InputLabel>Bidder</InputLabel>
                                <Select
                                    value={bidder}
                                    label="Bidder"
                                    onChange={(e) => setBidder(e.target.value as Player)}
                                >
                                    <MenuItem value="South">South</MenuItem>
                                    <MenuItem value="West">West</MenuItem>
                                    <MenuItem value="North">North</MenuItem>
                                    <MenuItem value="East">East</MenuItem>
                                </Select>
                            </FormControl>
                        </Grid>

                        <Grid item xs={12} md={6}>
                            <TextField
                              fullWidth
                              label="Bid Amount"
                              type="number"
                              value={bidAmount}
                              onChange={(e) => handleBidAmountChange(e.target.value)}
                              error={!!bidError}
                              helperText={bidError || 'Min 50, then by 1 till 60, by 5 till 99, then by 10'}
                            />
                        </Grid>

                        <Grid item xs={12}>
                            <Button
                                variant="contained"
                                fullWidth
                                onClick={handleSubmitBid}
                                disabled={!isValidBid}
                            >
                                Record Bid
                            </Button>
                        </Grid>
                    </Grid>
                </CardContent>
            </Card>
        </Container>
    )
}

export default HandPage
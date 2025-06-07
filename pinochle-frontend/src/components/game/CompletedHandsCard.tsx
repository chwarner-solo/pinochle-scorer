import React from "react";
import { Card, Table, TableHead, TableCell, TableBody, TableRow } from "@mui/material";
import type { Player, Suit } from "../../types/Game.ts";
import { Hand } from "../../types/Game.ts";

const hands  = [
    {hand_id: "1", bidder: Player.North, bid_amount: 51, trump: Suites.Spades, us_meld: 24, them_meld: 32, us_tricks: 27, them_tricks: 23, us_total: 51, them_total: 55, dealer: Player.South},
    {hand_id: "2", bidder: Player.North, bid_amount: 51, trump: Suites.Hearts, us_meld: 24, them_meld: 32, us_tricks: 27, them_tricks: 23, us_total: 51, them_total: 55, dealer: Player.West},
];

const CompletedHandsCard = (props) => {
    const completedHands = props.hands ?? hands;

    if (!completedHands.length) return null;

    return (
        <Card
            size={{ width: { xs: '100%', sm: 500, md: 600 }}}
            style={{
                maxWidth: 600,
                marginBottom: 16,
                width: '100%',
                overflowX: 'auto',
            }}
            elevation={2}>
            <Table size="small">
                <TableHead>
                    <TableRow>
                        <TableCell> Hand #</TableCell>
                        <TableCell> US</TableCell>
                        <TableCell> Them</TableCell>
                        <TableCell> Bid</TableCell>
                        <TableCell> Who</TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {completedHands.map((hand : Hand, idx) => (
                        <React.Fragment key={hand.hand_id || idx}>
                            <TableRow>
                                <TableCell rowSpan={3} align="center" style={{ verticalAlign: "middle"}}>
                                    {hand.number}
                                </TableCell>
                                <TableCell>{hand.us_meld ?? "--"}</TableCell>
                                <TableCell>{hand.them_meld ?? "--"}</TableCell>
                                <TableCell rowSpan={3} align="center" style={{ verticalAlign: "middle"}}>
                                    {hand.bid} - {hand.bidder} - {hand.trump ?? "--"}
                                </TableCell>
                                <TableCell rowSpan={3} align="center" style={{ verticalAlign: "middle"}}>
                                    {hand.dealer ?? '--'}
                                </TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>{hand.us_tricks ?? "--"}</TableCell>
                                <TableCell>{hand.them_tricks ?? "--"}</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell>{hand.us_total ?? "--"}</TableCell>
                                <TableCell>{hand.them_total ?? "--"}</TableCell>
                            </TableRow>
                        </React.Fragment>
                    ))}
                </TableBody>
            </Table>
        </Card>
    )
}

export default CompletedHandsCard;
import type {BidFormData, TrumpFormData, FormData} from "../types/form_types";
import type { HandState } from "../types/Game";

export type ValidationErrors = { [field: string]: string | undefined}

export const validateBid= (form: BidFormData): ValidationErrors => {
    const errors: ValidationErrors = {};

    if (!form.player) errors.player = "Select a player for the bid.";
    const bid = getBidError(form.bid);
    if (bid) errors.bid = bid;

    return errors;
}

export const validateTrump = (form: TrumpFormData): ValidationErrors => {
    const errors: ValidationErrors = {};
    if (!form.trump) errors.trump = "Select a trump.";
    return errors;
}

export const validateMeld = (): ValidationErrors => {
    // No errors or user notification for meld < 20; normalization happens silently
    return {};
}

export const validateTricks = (form: { us_tricks?: number; them_tricks?: number }): ValidationErrors => {
    const errors: ValidationErrors = {};
    const us = Number(form.us_tricks);
    const them = Number(form.them_tricks);

    // If both are blank/zero, fail
    if ((!us || isNaN(us)) && (!them || isNaN(them))) {
        errors.us_tricks = "Enter at least one team's tricks.";
        errors.them_tricks = "Enter at least one team's tricks.";
        return errors;
    }

    let finalUs = us;
    let finalThem = them;

    // If either side is zero, infer the other
    if (us === 0) {
        finalUs = 50 - them;
    }
    if (them === 0) {
        finalThem = 50 - us;
    }

    // Must sum to 50
    if (finalUs + finalThem !== 50) {
        errors.us_tricks = "US and THEM tricks must add up to 50.";
        errors.them_tricks = "US and THEM tricks must add up to 50.";
        return errors;
    }

    // Each must be between 0 and 50
    if (finalUs < 0 || finalUs > 50) {
        errors.us_tricks = "US tricks must be between 0 and 50.";
    }
    if (finalThem < 0 || finalThem > 50) {
        errors.them_tricks = "THEM tricks must be between 0 and 50.";
    }

    return errors;
}

export type ValidatorMap = {
    [K in HandState]: (form: FormData[K]) => ValidationErrors
};

export const validationMap: ValidatorMap =  {
    NoHand: () => ({}),
    "WaitingForBid": validateBid,
    WaitingForTrump: validateTrump,
    WaitingForMeld: validateMeld,
    WaitingForTricks: validateTricks,
    Completed: () => ({}),
    NoMarriage: validateMeld
}

function getBidError(amount: number): string | undefined {
    if (isNaN(amount)) return 'Please enter a number';
    if (amount < 50) return 'Bid must be at least 50';
    if (amount >= 60 && amount <= 99 && amount % 5 !== 0) return 'Bid must be a multiple of 5 when between 60 and 99';
    if (amount >= 100 && amount % 10 !== 0)
        return 'Bids 100+ must be multiples of 10';
    return undefined;
}
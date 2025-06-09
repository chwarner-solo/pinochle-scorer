import type {BidFormData, MeldFormData, TrumpFormData, FormData, CompletedFormData} from "../types/form_types";
import type { HandState } from "../types/Game";

export type ValidationErrors = { [field: string]: string | undefined}

export const validateBid= (form: BidFormData): ValidationErrors => {
    const errors: ValidationErrors = {};

    if (!form.player) errors.player = "Select a player for the bid.";
    let bid = getBidError(form.bid);
    if (bid) errors.bid = bid;

    return errors;
}

export const validateTrump = (form: TrumpFormData): ValidationErrors => {
    const errors: ValidationErrors = {};
    if (!form.trump) errors.trump = "Select a trump.";
    return errors;
}

export const validateMeld = (form: MeldFormData): ValidationErrors => {
    const errors: ValidationErrors = {};
    if (form.us_meld < 20 && form.us_meld !== 0) errors.us_meld = "US meld must be 20 or higher.";
    if (form.them_meld < 20 && form.them_meld !== 0) errors.them_meld = "THEM meld must be 20 or higher.";
    return errors;
}

export const validateTricks = (form: any): ValidationErrors => {
    const errors: ValidationErrors = {};
    const us = form.us_tricks;
    const them = form.them_tricks;

    // 1. If both us and them are defined, make sure they add up to 50
    if (
        typeof us === "number" &&
        typeof them === "number" &&
        us !== 0 &&
        them !== 0
    ) {
        if (us +them !== 50) {
            errors.us_tricks = "US and THEM tricks must add up to 50.";
            errors.them_tricks = "US and THEM tricks must add up to 50.";
            return errors;
        }
    }

    // 2. If either us or them is defined, make sure it's between 20 and 50

    if (us !== undefined && us !== 0 && (us < 20 || us > 50)) {
        errors.us_tricks = "US tricks must be between 20 and 50.";
    }

    if (them !== undefined && them !== 0 && (them < 20 || them > 50)) {
        errors.them_tricks = "THEM tricks must be between 20 and 50.";
    }

    return errors;
}

export type ValidatorMap = {
    [K in HandState]: (form: FormData[K]) => ValidationErrors
};

export const validationMap: ValidatorMap =  {
    "WaitingForBid": validateBid,
    WaitingForTrump: validateTrump,
    WaitingForMeld: validateMeld,
    WaitingForTricks: validateTricks,
    Completed: (_form: CompletedFormData) => ({}),
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
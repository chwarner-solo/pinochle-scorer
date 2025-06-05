import { gamepi as api } from '../api';

const stateActionMap = {
    WaitingForBid: api.startHand,
    WaitingForTrump: api.declareTrump,
    WaitingForMeld: api.declareMeld,
    WaitingForTricks: api.declareTricks
};

export const useHandProgression = (gameId: string, hand?: Hand) => {
    const [handData, setHandData] = useState<HandFormData | null>(null);
    const [isSubmitting, setIsSubmitting] = useState(false);

    const onSubmit = useCallback(
        async (form: HandFormData) => {
            const apiMethod = stateActionMap[form.state];
            setIsSubmitting(true);
            try {
                const updatedHand = await apiMethod(gameId, form.data);
                setHandData(updatedHand);
            } finally {
                setIsSubmitting(false);
            }
        }, [gameId, hand?.state]);

    return { handData, isSubmitting, onSubmit };
}
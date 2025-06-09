import React, { useState } from 'react';
import TitleCard from '../components/TitleCard';
import GameStatusCard from '../components/GameStatusCard';
import HandEntryCard from '../components/HandEntryCard';
import HandsTableCard from '../components/HandsTableCard';
import {useGame} from "../hooks/useGame.ts";
import {realGameApi} from "../services/api.ts";
import GameHandAdminPanel from "../components/GameHandAdminPanel.tsx";

const CARD_COMPONENTS = [
  HandEntryCard,
];

const Index: React.FC = () => {
  const [currentCard, setCurrentCard] = useState(0);
  const CardComponent = CARD_COMPONENTS[currentCard];

  const gameStatus = useGame(realGameApi);

  return (
    <div className="min-h-screen w-full h-full bg-gray-100 flex flex-col py-8 px-2 md:px-8">
      <div className="w-full max-w-screen-md mx-auto">
        <TitleCard />
        <div className="flex-1 flex flex-col w-full">
          <GameStatusCard {...gameStatus} />
        </div>
        <div className="flex-1 flex flex-col w-full">
          <CardComponent {...gameStatus} />
          <HandsTableCard />
          <GameHandAdminPanel {...gameStatus} />
        </div>
          <div className="flex justify-between mt-4">
            {/* ...buttons... */}
          </div>
        </div>
    </div>
  );
};

export default Index;

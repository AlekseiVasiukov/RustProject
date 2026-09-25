import { Component, inject,  computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router} from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';


@Component({
  selector: 'app-unit-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './unit-list.html',
  styleUrl: './unit-list.scss',
})
import { Component, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';

@Component({
  selector: 'app-unit-list',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './unit-list.html',
  styleUrl: './unit-list.scss'
})
export class UnitList {
  session = inject(GameSessionService);
  router = inject(Router);

  //exposing signals for usage in template
availableFactions = this.session.availableFactions;
availableUnittypes = this.session.availableUnitTypes;
ourFactionIndex = this.session.ourFactionIndex;
opponentFactionIndex = this.session.opponentFactionIndex;
ourWarriorTypes = this.session.ourWarriorTypes;


//we validating unittypes not everyone null, then allowing confirmation.
isValidSelection = this.session.isValidSelection;

//getting unit type name for particular faction index and unit index
getUnitTypeName(factionIndex: number, unitTypeIndex: number | null): string{
  if (unitTypeIndex === null) return 'None';
  const types = this.availableUnittypes()[factionIndex];
  return types?.[unitTypeIndex] ?? 'None';
}

//changing our faction
onOurFactionChange(event: Event){
  const index = parseInt((event.target as HTMLSelectElement).value, 10);
  this.session.ourFactionIndex.set(index);
}

//changing oppos faction
onOpponentFactionChange(event: Event){
  const index = parseInt((event.target as HTMLSelectElement).value, 10);
  this.session.opponentFactionIndex.set(index);
}

//changing unit types
onWarriorTypeChange(slotIndex: number, event: Event){
  const value = (event.target as HTMLSelectElement).value;
  const newIndex = value === 'null' ? null : parseInt(value, 10);

  const current = [...this.session.ourWarriorTypes()];
  current[slotIndex] = newIndex;

  //validation
  const allOthersNull = current.every((v, i) => i !== slotIndex && v === null);
  if (newIndex === null && allOthersNull) {
    return;
  }

  this.session.ourWarriorTypes.set(current);
}

onConfirmSelection(){
  if (!this.session.isValidSelection()) return;
  this.session.confirmSelection();
  this.router.navigate(['/enemies']);
}
}


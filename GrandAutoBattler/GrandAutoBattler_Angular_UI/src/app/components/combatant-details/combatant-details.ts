import { Component, inject, input, output } from '@angular/core';
import type { Combatant } from '../../core/models/game.model';
// import {  } from '@angular/core';

@Component({
  selector: 'app-combatant-details',
  standalone: true,

  imports: [],
  templateUrl: './combatant-details.html',
  styleUrl: './combatant-details.scss',
})
export class CombatantDetails {
  combatant = input.required<Combatant | null>();
  closed = output<void>();
  // selectedCombatant = signal<Combatant | null>(null);
}

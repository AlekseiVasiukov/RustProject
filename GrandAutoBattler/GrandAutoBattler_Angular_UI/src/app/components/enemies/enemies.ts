import { Component, inject, signal, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { GameSessionService } from '../../core/services/game-session.service';
import { Router } from '@angular/router';
import type { Combatant } from '../../core/models/game.model';
import { UnitImageService } from '../../core/services/unit-image.service';
import { CombatantDetails } from '../combatant-details/combatant-details';
@Component({
  selector: 'app-enemies',
  standalone: true,
  imports: [CommonModule, CombatantDetails],
  templateUrl: './enemies.html',
  styleUrl: './enemies.scss',
})
export class Enemies {
  session = inject(GameSessionService);
  imageService = inject(UnitImageService);
  // combatantDetails
  router = inject(Router);
  selectedCombatant = signal<Combatant | null>(null);
  selectedConbatantSide = signal<0 | 1>(0);

  //our combatants (side 0)
  ourCombatants = computed(() => {
    const result = this.session.gameResult();
    return result?.generated_combatants?.[0] ?? [];
  });

  //opponents (side 1)
  enemyCombatants = computed(() => {
    const result = this.session.gameResult();
    return result?.generated_combatants?.[1] ?? [];
  });

  //get display name for combatants
  getFullName(combatant: Combatant): string {
    const first = combatant.first_name ?? '';
    const last = combatant.last_name ?? '';
    return `${first} ${last}`.trim() || 'Unknown';
  }

  ngOnInit() {
    this.imageService.updateConfig({ basePath: '/assets/images/128px' });
  }
  getCombatantImage = (combatant: Combatant, side: 0 | 1): string => {
    return this.imageService.getCombatantImagePath(combatant.unit_type_name, side);
  };

  //get pretty name of unit
  getUnitType(combatant: Combatant): string {
    return combatant.unit_type_pretty_name || combatant.unit_type_name;
  }

  goToBattleJournal() {
    this.router.navigate(['/battle-journal']);
  }

  onCombatantClick(combatant: Combatant, side: 0 | 1): void {
    console.log('type in enemies is', combatant.unit_type_name);
    console.log('side in enemies is', side);

    this.selectedConbatantSide.set(side);
    this.selectedCombatant.set(combatant);
  }
}

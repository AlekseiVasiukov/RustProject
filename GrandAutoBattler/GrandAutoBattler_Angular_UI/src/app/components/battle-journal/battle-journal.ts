import { Component, inject, computed } from '@angular/core';
import { CommonModule } from '@angular/common';
import { Router } from '@angular/router';
import { GameSessionService } from '../../core/services/game-session.service';
import type { Combatant, BattleJournalEvent } from '../../core/models/game.model';

@Component({
  selector: 'app-battle-journal',
  standalone: true,
  imports: [CommonModule],
  templateUrl: './battle-journal.html',
  styleUrl: './battle-journal.scss',
})
export class BattleJournal {
  session = inject(GameSessionService);
  router = inject(Router);

  //Current page index of battle journal
  currentPage = 0;

  //All battle events
  events = computed(() => {
    return this.session.gameResult()?.battle_journal?.events ?? [];
  });

  //Current event (we display this one)
  currentEvent = computed(() => {
    return this.events()[this.currentPage];
  });

  totalEvents = computed(() => this.events().length);

  //our combatants (side 0)
  ourCombatants = computed(() => {
    return this.session.gameResult()?.generated_combatants?.[0] ?? [];
  });

  enemyCombatants = computed(() => {
    return this.session.gameResult()?.generated_combatants?.[1] ?? [];
  });

  attackerId = computed(() => {
    const evt = this.currentEvent();
    return evt?.cmd?.type === 'Attack' ? evt.cmd.src : '';
  });

  victimId = computed(() => {
    const evt = this.currentEvent();
    return evt?.cmd?.type === 'Attack' ? evt.cmd.dst : '';
  });

  //navigating to results
  goToResults() {
    this.router.navigate(['/results']);
  }

  previousPage() {
    if (this.currentPage > 0) {
      this.currentPage--;
    }
  }
  nextPage() {
    if (this.currentPage < this.totalEvents() - 1) {
      this.currentPage++;
    }
  }

  goToStart() {
    this.currentPage = 0;
  }

  goToEnd() {
    this.currentPage = this.totalEvents() - 1;
  }

  //Kleyborad input navigation

  handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'ArrowLeft') {
      this.previousPage();
    } else if (event.key === 'ArrowRight') {
      this.nextPage();
    } else if (event.key === 'ArrowUp') {
      this.goToStart();
    } else if (event.key === 'ArrowDown') {
      this.goToEnd();
    }
  }

  getFullName(c: Combatant): string {
    const first = c.first_name ?? '';
    const last = c.last_name ?? '';
    return `${first} ${last}`.trim() || 'Unknown';
  }

  getCombatantHightlighClass(c: Combatant): string {
    if (c.current_health_points <= 0) return 'dead';
    if (c.id === this.attackerId()) return 'attacker';
    if (c.id === this.victimId()) return 'victim';
    return 'alive';
  }
}

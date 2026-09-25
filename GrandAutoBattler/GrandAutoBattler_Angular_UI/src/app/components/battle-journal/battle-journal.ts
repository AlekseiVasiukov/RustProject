import { Component, inject, computed, signal } from '@angular/core';
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
  currentPage = signal(0);

  //All battle events
  events = computed(() => {
    return this.session.gameResult()?.battle_journal?.events ?? [];
  });

  //Current event (we display this one)
  currentEvent = computed(() => {
    return this.events()[this.currentPage()];
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
    var evt = this.currentEvent();

    const cmdData = evt?.cmd['Attack'];
    console.log('Current Cmd Data:', cmdData);
    return cmdData?.src ?? '';
  });

  victimId = computed(() => {
    const evt = this.currentEvent();
    const cmdData = evt?.cmd['Attack'];

    console.log('Current Cmd Data:', cmdData);

    return cmdData?.dst ?? '';
  });

  //navigating to results
  goToResults() {
    this.router.navigate(['/results']);
  }

  previousPage() {
    if (this.currentPage() > 0) {
      this.currentPage.update((v) => v - 1);
      console.log(this.currentPage());
    }
  }
  nextPage() {
    if (this.currentPage() < this.totalEvents() - 1) {
      this.currentPage.update((v) => v + 1);
      console.log(this.currentPage());
    }
  }

  goToStart() {
    this.currentPage.set(0);
    console.log(this.currentPage());
  }

  goToEnd() {
    this.currentPage.set(this.totalEvents() - 1);
    console.log(this.currentPage());
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
    var result = '';
    if (c.current_health_points <= 0) result = 'dead';
    if (c.id === this.attackerId()) return 'attacker';
    if (c.id === this.victimId()) return 'victim';
    result = 'alive';

    console.log(result);
    return result;
  }
}

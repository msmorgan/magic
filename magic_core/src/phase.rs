#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BeginningStep {
    Untap,
    Upkeep,
    Draw,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CombatStep {
    BeginCombat,
    DeclareAttackers,
    DeclareBlockers,
    FirstCombatDamage,
    CombatDamage,
    EndCombat,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EndingStep {
    End,
    Cleanup,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Step {
    Beginning(BeginningStep),
    FirstMain,
    Combat(CombatStep),
    SecondMain,
    Ending(EndingStep),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Beginning,
    FirstMain,
    Combat,
    SecondMain,
    Ending,
}

impl Step {
    pub fn iterator() -> impl Iterator<Item = Step> {
        const STEPS: [Step; 13] = [
            Step::Beginning(BeginningStep::Untap),
            Step::Beginning(BeginningStep::Upkeep),
            Step::Beginning(BeginningStep::Draw),
            Step::FirstMain,
            Step::Combat(CombatStep::BeginCombat),
            Step::Combat(CombatStep::DeclareAttackers),
            Step::Combat(CombatStep::DeclareBlockers),
            Step::Combat(CombatStep::FirstCombatDamage),
            Step::Combat(CombatStep::CombatDamage),
            Step::Combat(CombatStep::EndCombat),
            Step::SecondMain,
            Step::Ending(EndingStep::End),
            Step::Ending(EndingStep::Cleanup),
        ];

        STEPS.iter().cloned()
    }

    pub fn phase(self) -> Phase {
        match self {
            Step::Beginning(_) => Phase::Beginning,
            Step::FirstMain => Phase::FirstMain,
            Step::Combat(_) => Phase::Combat,
            Step::SecondMain => Phase::SecondMain,
            Step::Ending(_) => Phase::Ending,
        }
    }

    pub fn has_priority(self) -> bool {
        match self {
            Step::Beginning(BeginningStep::Untap) | Step::Ending(EndingStep::Cleanup) => false,
            _ => true,
        }
    }
}

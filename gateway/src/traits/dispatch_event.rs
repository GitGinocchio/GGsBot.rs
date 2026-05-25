use twilight_model::{gateway::event::DispatchEvent, id::{Id, marker::UserMarker}};

pub trait DispatchEventExt {
    fn user_id(&self) -> Option<Id<UserMarker>>;
}

impl DispatchEventExt for DispatchEvent {
    fn user_id(&self) -> Option<Id<UserMarker>> {
        match self {
            // Eventi legati ai messaggi e interazioni
            DispatchEvent::MessageCreate(msg) => Some(msg.author.id),
            DispatchEvent::MessageUpdate(msg) => Some(msg.author.id),
            DispatchEvent::InteractionCreate(inter) => inter.author_id(), // Fornito nativamente da Twilight
            
            // Eventi legati ai membri del server
            DispatchEvent::MemberAdd(member) => Some(member.user.id),
            DispatchEvent::MemberUpdate(member) => Some(member.user.id),
            DispatchEvent::MemberRemove(member) => Some(member.user.id),
            DispatchEvent::PresenceUpdate(pres) => Some(pres.user.id()),
            DispatchEvent::TypingStart(typing) => Some(typing.user_id),
            
            // Eventi vocali
            DispatchEvent::VoiceStateUpdate(state) => Some(state.user_id),
            
            // Reazioni e Polls
            DispatchEvent::ReactionAdd(react) => Some(react.user_id),
            DispatchEvent::ReactionRemove(react) => Some(react.user_id),
            DispatchEvent::MessagePollVoteAdd(vote) => Some(vote.user_id),
            DispatchEvent::MessagePollVoteRemove(vote) => Some(vote.user_id),
            
            // Auto-moderazione
            DispatchEvent::AutoModerationActionExecution(exe) => Some(exe.user_id),

            // Inviti e Audit Log
            DispatchEvent::InviteCreate(invite) => invite.inviter.as_ref().map(|u| u.id),
            DispatchEvent::GuildAuditLogEntryCreate(entry) => entry.user_id,

            // Per tutti gli altri eventi dove l'attore non è presente direttamente nel payload
            _ => None,
        }
    }
}
namespace GrandAutoBattler_Blazor.Models
{
    public record NewGameIdResponse()
    {
        public string game_id { get; init; } = string.Empty;
        public bool success { get; init; }
        public string timestamp { get; init; } = string.Empty;
        public string? error { get; init; }

    }
}

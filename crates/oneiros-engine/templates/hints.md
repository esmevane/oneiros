{% if !hints.is_empty() %}

## Hints
{% for hint in hints %}
- {{ hint.level.emphasis() }} `{{ hint.action.hint() }}` — {{ hint.intent }}
{% endfor %}
{% endif %}

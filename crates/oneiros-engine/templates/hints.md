{% if !hints.is_empty() %}

## Hints
{% for hint in hints %}
- **{{ hint.level }}** `{{ hint.action }}` — {{ hint.intent }}
{% endfor %}
{% endif %}

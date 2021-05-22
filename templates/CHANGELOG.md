## {{ version | default(value="[unreleased]") }}
{% for group, commits in commits | group_by(attribute="group") %}
### {{ group | capitalize}}
{% for commit in commits %}
- {{ commit.message | capitalize }}{% endfor %}
{% endfor %}
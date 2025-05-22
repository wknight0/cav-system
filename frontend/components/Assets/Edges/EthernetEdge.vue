<script>
    export const createEthernetEdge = (params, edgeId, color) => {
        return {
            id: edgeId,
            ...params,
            type: 'straight',
            connection_type: 'ethernet',
            animated: false,
            style: {
                stroke: 'var(--tertiary-color)',
                strokeWidth: 2,
            },
        };
    };
</script>

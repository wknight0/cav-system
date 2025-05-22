<script>
    export const createWirelessEdge = (params, edgeId, color) => {
        return {
            id: edgeId,
            ...params,
            type: 'straight',
            connection_type: 'wireless',
            animated: false,
            style: {
                stroke: 'var(--tertiary-color)',
                strokeDasharray: '8 8',
                strokeWidth: 8,
            },
        };
    };
</script>
